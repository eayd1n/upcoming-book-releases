# upcoming-book-releases

Get informed about upcoming book releases (in german language) of your favourite authors.

## Build

To build the upcoming-book-releases application, just execute

```bash
cargo build --release
```

The application was built and tested with Rust v1.77.1.

## Usage

First of all, you need a simple text file containing your authors. The authors must be listed in the
format <surname, first name>. A sample file could look like this:

```
Follett, Ken
Glukhovsky, Dmitry
Greaney, Mark
Hurwitz, Gregg
Katzenbach, John
Kepler, Lars
King, Stephen
```

The authors do not need to be sorted alphabetically because later on we sort the potential upcoming releases by date.

Now you are ready to use the application. The Mnemonics:

```bash
Usage: upcoming_book_releases [OPTIONS]

Options:
  -a, --authors-file <AUTHORS_FILE>  Path to the file containing your authors [default: /home/authors]
  -d, --dest-release <DEST_RELEASE>  Destination path the release file has to be stored [default: /home]
  -r, --release-file <RELEASE_FILE>  Name of the release file [default: releases]
  -l, --loglevel <LOGLEVEL>          Log level (off, warn, error, info, debug, trace) [default: info]
  -h, --help                         Print help
  -V, --version                      Print version
```

Besides the path to your author's file, you also need to specify the destination path where and
under which name the release file has to be stored.

Exemplary usage:

```bash
upcoming_book_releases -a /home/workspace/authors -d /home -r upcoming-releases
```

The execution of the example above reads the author's file from "/home/workspace/authors" and stores
the upcoming releases in "/home/upcoming-releases".

An exemplary release file could look like this:

```
Upcoming Book Releases

28. Februar 2024
-----------------------------------------------------------------------------------
Gregg Hurwitz - "Batman - The Dark Knight von David Finch (Deluxe Edition)"

1. März 2024
-----------------------------------------------------------------------------------
Sebastian Fitzek - "Flugangst 7A"

13. März 2024
-----------------------------------------------------------------------------------
Sam Feuerbach - "Der Grauzorn / Minen der Macht Bd.3"

21. März 2024
-----------------------------------------------------------------------------------
Jussi Adler-Olsen - "Verraten / Carl Mørck. Sonderdezernat Q Bd.10"
Jussi Adler-Olsen - "NATRIUM CHLORID / Carl Mørck. Sonderdezernat Q Bd.9"

15. April 2024
-----------------------------------------------------------------------------------
David Baldacci - "Open Fire"

23. April 2024
-----------------------------------------------------------------------------------
Don Winslow - "City of Dreams / City on Fire Bd.2"

2. Mai 2024
-----------------------------------------------------------------------------------
Sebastian Fitzek - "Survival Guide für den Elternabend"

13. Mai 2024
-----------------------------------------------------------------------------------
Jeffery Deaver - "Vatermörder / Colter Shaw Bd.3"
Jeffery Deaver - "Rachejäger / Colter Shaw Bd.4"

14. Mai 2024
-----------------------------------------------------------------------------------
Mark Greaney - "The Gray Man - Undercover in Syrien"

21. Mai 2024
-----------------------------------------------------------------------------------
Stephen King - "Ihr wollt es dunkler"
Don Winslow - "City in Ruins / City on Fire Bd.3"

30. Mai 2024
-----------------------------------------------------------------------------------
Chris Carter - "Der Totenarzt"

28. Juni 2024
-----------------------------------------------------------------------------------
David Baldacci - "Finstere Lügen"
David Baldacci - "Gefährliches Komplott"

1. Juli 2024
-----------------------------------------------------------------------------------
John Katzenbach - "Die Komplizen. Fünf Männer, fünf Mörder, ein perfider Plan"

26. Juli 2024
-----------------------------------------------------------------------------------
Ken Follett - "Never - Die letzte Entscheidung"
Ken Follett - "Der dritte Zwilling"
Ken Follett - "Die Kinder von Eden"

1. August 2024
-----------------------------------------------------------------------------------
John Katzenbach - "Die Familie / Dr. Frederick Starks Bd.3"

30. August 2024
-----------------------------------------------------------------------------------
Ethan Cross - "Im Labyrinth der Rache"
Andreas Eschbach - "Der schlauste Mann der Welt"

9. September 2024
-----------------------------------------------------------------------------------
Marc Elsberg - "°C - Celsius"

30. September 2024
-----------------------------------------------------------------------------------
Simon Beckett - "Knochenkälte / David Hunter Bd.7"
```
