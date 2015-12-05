my $boxes = slurp 'input.txt';
my $area = 0;

for $boxes.comb(/$<l>=[\d]+x$<w>=[\d]+x$<h>=[\d]+/, :match<True>) -> $box {
  my $l = $box<l>.Int;
  my $w = $box<w>.Int;
  my $h = $box<h>.Int;

  $area += 2 * ($l*$w + $w*$h + $h*$l);
  $area += min($l*$w, $w*$h, $h*$l);
}

say 'The elves need to order ' ~ $area ~ ' square feet of wrapping paper';
