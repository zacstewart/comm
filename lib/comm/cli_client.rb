require 'curses'
require 'set'
require 'comm/cli_client/peer_list'

module Comm
  class CliClient
    def initialize(node)
      @node = node
      @node.attach_client(self)

      Curses.noecho
      @window = Curses::Window.new(0, 0, 0, 0)
      @transcript = @window.subwin(Curses.lines - 1, Curses.cols - 42, 0, 0)
      @transcript.scrollok(true)
      @transcript.setpos(@transcript.maxy - 1, 0)
      @input = @window.subwin(1, 0, Curses.lines - 1, 0)
      @peers = PeerList.new(@window)
    end

    def add_message(message_payload)
      @transcript.addstr("<#{message_payload.sender}> #{message_payload.text}")
      @transcript.scroll
      @transcript.setpos(@transcript.cury, 0)
      @transcript.refresh
    end

    def add_peer(peer)
      @peers.add(peer)
    end

    def remove_peer(peer)
      @peers.remove(peer)
    end

    def stop
      @stopped = true
    end

    def run
      @stopped = false
      buffer = ''

      @input.keypad = true
      loop do
        chr = @input.getch
        case chr
        when Curses::Key::UP
          @peers.select_previous
        when Curses::Key::DOWN
          @peers.select_next
        when Curses::Key::BACKSPACE, 127
          buffer.chop!
        when Curses::Key::ENTER, 10
          @input.clear
          @input.refresh
          @input.setpos(0, 0)
          node.deliver_chat(buffer, to: @peers.selected)
          buffer.clear
        when String
          buffer << chr
        end

        @input.clear
        @input.addstr(buffer)
        @input.refresh
        break if @stopped
      end
    end

    private

    attr_reader :node
  end
end
