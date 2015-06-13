#!/usr/bin/perl
## Written by DGC
## Fri,  4/05/2012

## problem5
## Description -

# 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

# What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

use Getopt::Long;
use Pod::Usage;
use strict;
use warnings;
use Cwd;
use File::Basename;
use Switch;

## Options

my $help;

Getopt::Long::Configure ('bundling');
GetOptions(
           "h|help" => \$help
           );
if ($help) {
  pod2usage({
    -verbose => 1,
    -exitval => 0
    });
};

## Main

my $result;
my $step = 2520;
my $i = 2520;

for (my $highest = 10; $highest < 21; $highest++) {
  while (!result($i,$highest)) {
    $i += $step;
  }
  # print "Biggest common multiple of 1..$highest : $i\n";
  $step = $i;
}

print "Result : $i\n";


#-------------------------------------------------
sub result {
my ($x,$max) = @_;

for (2..$max) {
  if ($x % $_) {
    return 0
  }
}

return 1;
}

__END__
=head1 NAME

problem5 - 

=head1 SYNOPSIS

problem5 [-h]

=head1 OPTIONS

=over 1

=item B <-h>

Print help message and exit successfully.

