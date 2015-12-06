my $words = slurp 'input.txt';
my $nice = 0;

for $words.words -> $word {
  # Does the word contain any of the forbidden combinations?
  next if $word ~~ /ab||cd||pq||xy/;
  # Does the word contain at least one letter that appears twice in a row?
  next if $word !~~ /(.)$0/;
  # Does the word contain at least 3 vowels?
  next if ($word.comb(/./).grep: /<[aeiou]>/).elems < 3;
  $nice += 1;
}

say 'Santa\'s list contains ' ~ $nice ~ ' nice words';
