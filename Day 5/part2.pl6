my $words = slurp 'input.txt';
my $nice = 0;

for $words.words -> $word {
  # Does the word contain at least one pair of letters twice?
  next if $word !~~ /(..).*$0/;
  # Does the word contain at least one letter that repeats with another inbetween?
  next if $word !~~ /(.).$0/;
  $nice += 1;
}

say 'Santa\'s list contains ' ~ $nice ~ ' nice words';
