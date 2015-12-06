my $moves = slurp 'input.txt';
my $pos_santa = [0, 0];
my $pos_robo = [0, 0];
my %visited;

# Visit the start position immediately
%visited{$pos_santa.Str} = True;

for $moves.comb(/./).kv -> $index, $move {
  my $pos := $index %% 2 ?? $pos_santa !! $pos_robo;
  given $move {
    when '^' { $pos[1] += 1 }
    when 'v' { $pos[1] -= 1 }
    when '>' { $pos[0] += 1 }
    when '<' { $pos[0] -= 1 }
  }
  %visited{$pos.Str} = True;
}

say 'Santa and Robo-Santa deliver at least one present to ' ~ %visited.elems ~ ' houses';
