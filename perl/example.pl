
use stfl;

my $f = stfl::stfl_create("vbox \@style:bg=blue\n");
stfl::stfl_run($f, 0);
stfl::stfl_free($f);

stfl::stfl_return();

