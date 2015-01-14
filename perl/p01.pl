#!/usr/bin/perl
## Written by DGC
## Fri,  4/05/2012

# If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. 
# The sum of these multiples is 23.

# Find the sum of all the multiples of 3 or 5 below 1000.

use warnings;

## Main

my $limit = 1000;

print "Result -> " . (sum_divisble_by(3) + sum_divisble_by(5) - sum_divisble_by(15)) . "\n";

sub sum_divisble_by {
		my ($n) = @_;
		my ($limit, $result, $count) = (1000, 0, 0);
		
		while ($count * $n < $limit) {
				$result += $count * $n;
				$count += 1;
		}

		return $result;
}
