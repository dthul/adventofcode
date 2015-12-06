my $moves = slurp 'input.txt';
my $position = [0, 0];
my %visited;

# Visit the start position immediately
%visited{$position.Str} = True;

for $moves.comb(/./) -> $move {
  given $move {
    when '^' { $position[1] += 1 }
    when 'v' { $position[1] -= 1 }
    when '>' { $position[0] += 1 }
    when '<' { $position[0] -= 1 }
  }
  %visited{$position.Str} = True;
}

say 'Santa delivers at least one present to ' ~ %visited.elems ~ ' houses';
