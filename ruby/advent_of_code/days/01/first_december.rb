# frozen_string_literal: true

# The first day of december
class FirstDecember
  # Call this to solve the first problem of this day
  def self.first
    total = 0
    read_file.each do |line|
      first_number = 0
      last_number = 0
      is_first_number = true
      line.each_char do |char|
        next unless char.match?(/[[:digit:]]/)

        if is_first_number
          is_first_number = false
          first_number = char
        end
        last_number = char
      end
      number = first_number.to_s + last_number.to_s
      total += number.to_i
    end
    puts total
  end

  # Read the File and return it's content
  def self.read_file
    File.readlines("#{File.dirname(__FILE__)}/content.txt")
  end

  # Solves the second problem of the first day
  def self.second
  end
end
