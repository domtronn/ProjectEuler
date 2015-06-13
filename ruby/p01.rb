# If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. 
# The sum of these multiples is 23.

# Find the sum of all the multiples of 3 or 5 below 1000.

def sum_divisble_by(n)
  count = 0
  result = 0
  
  while count * n < 1000
    result += count * n
    count += 1
  end

  return result
end

p "Result -> #{sum_divisble_by(3) + sum_divisble_by(5) - sum_divisble_by(15)}"
