my $boxes = slurp 'input.txt';
my $length = 0;

for $boxes.comb(/$<l>=[\d]+x$<w>=[\d]+x$<h>=[\d]+/, :match<True>) -> $box {
  my $l = $box<l>.Int;
  my $w = $box<w>.Int;
  my $h = $box<h>.Int;

  $length += 2 * ($l + $w + $h - max($l, $w, $h));
  $length += $l * $w * $h;
}

say 'The elves need to order ' ~ $length ~ ' feet of ribbon';
