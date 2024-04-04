# 1.x.x

## Features

* Added link-time optimization (lto) which improves both build and run times and decreases size of
binary

# 1.0.1

## Bugfixes

* Number of releasing authors is determined correctly now

# 1.0.0

This is the initial release.

## Features

* Get the upcoming book releases of your favourite authors in german language

## (Potential) issues

* The information to get upcoming releases rely on the HTML structure of Weltbild.de. If the
structure changes, the scraper could possibly not work properly anymore.
* Depending on how the data is provided on the HTML layer, some parsing could lead to strange
results (e.g., if some title is encoded together with the author's name). Keep in mind the scraper
is not 100% perfect (but almost!)
