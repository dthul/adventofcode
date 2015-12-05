my $directions = slurp 'input.txt';
my $floor = 0;

for comb(/./, $directions).kv -> $index, $direction {
  if $direction eq '(' {
    ++$floor;
  } elsif $direction eq ')' {
    --$floor;
  }
  if $floor == -1 {
    say 'Santa first enters the basement on direction number ' ~ $index + 1;
    last;
  }
}
