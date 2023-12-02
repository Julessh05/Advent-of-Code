# frozen_string_literal: true

# The first day of december
class FirstDecember

  # Call this to solve the first problem of this day
  def self.first
  end

  # Read the File and return it's content
  def read_file
    file = File.open('content.txt')
    file_content = file.read
    file_content.lines.each { |line|

    }
  end

  # Solves the second problem of the first day
  def self.second

  end

end
