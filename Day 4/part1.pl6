use Digest::MD5;

my $input = 'iwrupvqb';
my $d = Digest::MD5.new;

for 1..^Inf -> $n {
  my $hash = $d.md5_hex($input ~ $n);
  if $hash.starts-with('00000') {
    say 'Santa can mine an AdventCoin with the number ' ~ $n;
    last;
  }
}
