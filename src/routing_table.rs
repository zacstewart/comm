use node_bucket;
use node_bucket::NodeBucket;
use address::{Address, Addressable, LENGTH};
use node::Node;

#[derive(Debug, PartialEq)]
pub enum InsertOutcome {
    Ignored,    // Currently just for ignoring self-node
    Inserted,   // Inserted new node
    Updated,    // Updated existing node
    Discarded   // Bucket is full
}

pub type InsertionResult = Result<InsertOutcome, String>;

#[derive(Debug)]
pub struct RoutingTable {
    k: usize,
    self_address: Address,
    routers: Vec<Box<Node>>,
    buckets: Vec<NodeBucket>
}

impl RoutingTable {
    pub fn new(k: usize, self_address: Address, routers: Vec<Box<Node>>) -> RoutingTable {
        let bucket = NodeBucket::new(k);
        RoutingTable {
            k: k,
            self_address: self_address,
            routers: routers,
            buckets: vec![bucket]
        }
    }

    pub fn insert(&mut self, node: Box<Node>) -> InsertionResult {
        if node.get_address() == self.self_address {
            return Ok(InsertOutcome::Ignored);
        }

        let index = self.bucket_for(&node.get_address());
        let mut bucket = self.buckets.remove(index);
        let self_address = self.self_address;

        if !self.buckets_maxed() && bucket.is_full() && bucket.covers(&self_address) {
            let (a, b) = bucket.split();
            self.buckets.insert(index, a);
            self.buckets.insert(index + 1, b);
            self.insert(node)
        } else {
            let status = bucket.insert(node);
            self.buckets.insert(index, bucket);
            match status {
                Ok(node_bucket::InsertOutcome::Inserted) => Ok(InsertOutcome::Inserted),
                Ok(node_bucket::InsertOutcome::Updated) => Ok(InsertOutcome::Updated),
                Ok(node_bucket::InsertOutcome::Discarded) => Ok(InsertOutcome::Discarded),
                Err(error) => Err(error)
            }
        }
    }

    pub fn find_node(&mut self, address: &Address) -> Option<&mut Box<Node>> {
        let index = self.bucket_for(address);
        let bucket = self.buckets.get_mut(index).unwrap();
        bucket.find_node(address)
    }

    pub fn nearest(&mut self) -> Vec<&mut Box<Node>> {
        let self_address = self.self_address;
        self.nearest_to(&self_address, true)
    }

    pub fn nearest_to(&mut self, address: &Address, include_routers: bool) -> Vec<&mut Box<Node>> {
        // TODO: this should walk buckets much more efficiently

        let mut candidates: Vec<&mut Box<Node>> = self.buckets
            .iter_mut()
            .flat_map(|b| b.get_nodes())
            .collect();

        candidates.sort_by_key(|n| n.get_address().distance_from(address));

        // chain on routers in case we don't have enough nodes yet
        if include_routers {
            candidates.into_iter().chain(self.routers.iter_mut()).take(self.k).collect()
        } else {
            candidates.into_iter().take(self.k).collect()
        }
    }

    pub fn questionable_nodes(&mut self) -> Vec<&mut Box<Node>> {
        // TODO: this should walk buckets much more efficiently

        self.buckets
            .iter_mut()
            .flat_map(|b| b.get_nodes())
            .filter(|n| n.is_questionable())
            .collect()
    }

    fn bucket_for(&self, address: &Address) -> usize {
        let (index, _) = self.buckets
            .iter()
            .enumerate()
            .find(|&(_, ref b)| b.covers(address))
            .unwrap();
        index
    }

    fn buckets_maxed(&self) -> bool {
        self.buckets.len() >= LENGTH
    }
}

#[cfg(test)]
mod tests {
    use address::{Address, Addressable};
    use super::{InsertOutcome, RoutingTable};
    use tests::TestNode;

    #[test]
    fn test_insert() {
        let self_node = Address::from_str("0000000000000000000000000000000000000000");
        let router = Box::new(TestNode::new(Address::null()));
        let mut table: RoutingTable = RoutingTable::new(2, self_node, vec![router]);
        table.insert(Box::new(TestNode::new(Address::from_str("0000000000000000000000000000000000000001")))).unwrap();
        table.insert(Box::new(TestNode::new(Address::from_str("ffffffffffffffffffffffffffffffffffffffff")))).unwrap();
        assert_eq!(table.buckets.len(), 1);

        // Splits buckets upon adding a k+1th node in the same space as self node
        table.insert(Box::new(TestNode::new(Address::from_str("fffffffffffffffffffffffffffffffffffffffe")))).unwrap();
        assert_eq!(table.buckets.len(), 2);
        table.insert(Box::new(TestNode::new(Address::from_str("7fffffffffffffffffffffffffffffffffffffff")))).unwrap();
        table.insert(Box::new(TestNode::new(Address::from_str("7ffffffffffffffffffffffffffffffffffffffe")))).unwrap();
        assert_eq!(table.buckets.len(), 3);

        // Replaces instead of duplicates existing nodes
        table.insert(Box::new(TestNode::new(Address::from_str("0000000000000000000000000000000000000001")))).unwrap();
        table.insert(Box::new(TestNode::new(Address::from_str("0000000000000000000000000000000000000001")))).unwrap();
        table.insert(Box::new(TestNode::new(Address::from_str("0000000000000000000000000000000000000001")))).unwrap();
        assert_eq!(table.buckets.len(), 3);

        // Disregards new nodes for full, non-self space buckets
        table.insert(Box::new(TestNode::new(Address::from_str("fffffffffffffffffffffffffffffffffffffffd")))).unwrap();
        table.insert(Box::new(TestNode::new(Address::from_str("fffffffffffffffffffffffffffffffffffffffc")))).unwrap();
        table.insert(Box::new(TestNode::new(Address::from_str("fffffffffffffffffffffffffffffffffffffffb")))).unwrap();
        assert_eq!(table.buckets.len(), 3);

        // Ignores self-node
        assert_eq!(table.insert(Box::new(TestNode::new(self_node))).unwrap(), InsertOutcome::Ignored);
        assert_eq!(table.buckets.len(), 3);
    }

    #[test]
    fn test_nearest_to() {
        let self_node = Address::from_str("0000000000000000000000000000000000000000");
        let router = Box::new(TestNode::new(Address::null()));
        let mut table: RoutingTable = RoutingTable::new(2, self_node, vec![router]);
        let addr_1 = Address::from_str("0000000000000000000000000000000000000001");
        let addr_2 = Address::from_str("7ffffffffffffffffffffffffffffffffffffffe");
        let addr_3 = Address::from_str("ffffffffffffffffffffffffffffffffffffffff");
        let node_1 = Box::new(TestNode::new(addr_1));
        let node_2 = Box::new(TestNode::new(addr_2));
        let node_3 = Box::new(TestNode::new(addr_3));
        table.insert(node_1).unwrap();
        table.insert(node_2).unwrap();
        table.insert(node_3).unwrap();

        {
            let nearest = table.nearest_to(&Address::from_str("fffffffffffffffffffffffffffffffffffffffd"), false);
            assert_eq!(nearest[0].get_address(), addr_3);
            assert_eq!(nearest[1].get_address(), addr_2);
        }
        {
            let nearest = table.nearest_to(&Address::from_str("0000000000000000000000000000000000000002"), false);
            assert_eq!(nearest[0].get_address(), addr_1);
            assert_eq!(nearest[1].get_address(), addr_2);
        }
    }
}
