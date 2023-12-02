# frozen_string_literal: true

module Days
  # The first day of december
  class FirstDecember
    # Call this to solve the first problem of this day
    def self.first
      total = 0
      read_file.each do |line|
        total += get_total(line)
      end
      puts total
    end

    def self.get_total(input)
      first_number = last_number = 0
      is_first_number = true
      input.each_char do |char|
        next unless char.match?(/[[:digit:]]/)

        first_number = char if is_first_number
        last_number = char
        is_first_number = false if is_first_number
      end
      number = first_number.to_s + last_number.to_s
      number.to_i
    end

    # Read the File and return it's content
    def self.read_file
      File.readlines("#{File.dirname(__FILE__)}/content.txt")
    end

    # Solves the second problem of the first day
    def self.second
    end
  end
end
