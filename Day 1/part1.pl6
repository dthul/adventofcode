my $directions = slurp 'input.txt';
my $floor = 0;

for comb(/./, $directions) -> $direction {
  if $direction eq '(' {
    ++$floor;
  } elsif $direction eq ')' {
    --$floor;
  }
}

say 'Santa should go to floor ' ~ $floor;
