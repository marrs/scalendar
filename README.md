Scalendar
=========

*Alpha software*

Scalendar is intended to add appointment and reminder functionality to the BSD
Calendar application. It is dependent on, and 100% compatible with, BSD
calendar.  As of yet this functionality has not been added.  What follows is a
wishlist.

BSD Calendar allows you to add events to a TSV (tab separated value) text file
with the following format:

```
DATE	Event description
```

An example event might be:

```
Jun 16	Danielle's birthday
```

Scalendar takes this format and augments it with additional data (format to be
confirmed).  Appointments can be added in the following way

```
Jun 16	9:30am 30m	Doctor's appointment
```

This sets a 30 minute appointment that starts at 9:30am. There should also be a
format for recurring appointments but this has not yet been explored.

Reminders can be set in the following way

```
Jun 16	-1d -1w -1m	Danielle's birthday
Jun 16	9:30am 30m	-30m -1h -1d -1w	Doctor's appointment
```

The first example reminds you of Danielle's birthday 1 day, 1 week, and 1 month
before the event.  The second example reminds you of your doctor's appoinment
30 minutes, 1 hour, 1 day, and 1 week ahead of schedule.

The details of how this will work are yet to be established.
