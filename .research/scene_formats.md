## Filename Encoding

- Permitted characters: `ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789._-`
- Consecutive punctuation is not allowed.

## Format Templates

| Name             | Template                                                                                                                                                            |
| ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Movie            | `<TITLE>.<YEAR>.<TAGS>.[LANGUAGE].<FORMAT>-<GROUP>`                                                                                                                 |
| EpisodeDaily     | `<SERIES>.[COUNTRY].<YEAR.MONTH.DAY>.[TITLE].<TAGS>.[LANGUAGE].<FORMAT>-<GROUP>`                                                                                    |
| EpisodeSeries    | `<SERIES>.[COUNTRY].[YEAR].<SEASON>.<EPISODE>.[-EPISODE2].[PART].[TITLE].<TAGS>.[LANGUAGE].<FORMAT>-<GROUP>`                                                        |
| EpisodeCrossover | `<SERIES>.[COUNTRY].[YEAR].<SEASON>.<EPISODE>.[PART].[TITLE].<SERIES2>.[COUNTRY2].[YEAR2].<SEASON2>.<EPISODE2>.[PART2].[TITLE2].<TAGS>.[LANGUAGE].<FORMAT>-<GROUP>` |
| EventNational    | `<COUNTRY>.<YEAR>.<EVENT>.FEED.<TAGS>.[LANGUAGE].<FORMAT>-<GROUP>`                                                                                                  |
| EventSport       | `<LEAGUE>.<YEAR>.[MONTH].[DAY].<EVENT>.<TAGS>.[LANGUAGE].<FORMAT>-<GROUP>`                                                                                          |
| EventMatch       | `<TITLE>.<YEAR>[-YEAR2].<ROUND>.<TAGS>.[LANGUAGE].<FORMAT>-<GROUP>`                                                                                                 |
| EventTournament  | `<TITLE>.<YEAR>.[TEAM.vs.TEAM2].<TAGS>.[LANGUAGE].<FORMAT>-<GROUP>`                                                                                                 |

## Rules

- Named directory arguments formatted inside <> must be included. Optional arguments formatted inside [] can be used in some cases.
- Mini-series parts must be at least 1 integer wide, and values used may extend past 9.
  e.g. Miniseries.Part.1, Miniseries.Part.10.
- Episode and seasonal numbering must be at least 2 integers wide, and values used may extend past 99.
  e.g. S01E99, S01E100, S101E01.
- Episode part refers to episodes, usually cartoons or animation, which split episodes into stories by different
  directors. Episode parts must be alphanumeric (A-Z, a-z, 0-9).
  e.g. The first episode from Season 2 of SpongeBob SquarePants is split into S02E01A/B, see:
  https://goo.gl/CVGXKu
- Season must be omitted if a series is not a mini-series and does not have seasons.
  e.g. One Piece must be tagged as One.Piece.E01.
- Episode title is optional.
- Tags refers to all permitted tags only, see section 17.
- Non-English releases must include the language tag. English releases must not include the language tag.
- Language tags must be the full name of the language. Abbreviations or language codes are not allowed.
  e.g. FRENCH, RUSSIAN, GERMAN.
- Format refers to the video source used.
  e.g. BluRay, TELECINE, HDDVD.
- Do not indicate the ripping, or encoding methods that were used. Use the NFO for any technical details.
- must include the production year.
- Different shows with the same title produced in different countries must have the ISO 3166-1 alpha 2 country code in
  the show name.
- Except for UK shows, which must use UK, not GB.
- This rule does not apply to an original show, only shows that succeed the original.
  e.g. The.Office.S01E01 and The.Office.US.S01E01.
- Different shows with the same title produced in the same country which begin in different years must have the year
  of the first season in the directory.
- The year is not required for the show broadcasted first.
  e.g. Second.Chance.S01E01 and Second.Chance.2016.S01E01.
- Different shows with the same titles produced in the same country which begin in different years must have the
  ISO-3166-1 alpha 2 country code followed by the year of the first season in the directory.
- See rules 18.8 and 18.9 for country code and year explanations.
  e.g. Wanted.S01E01 (2005), Wanted.AU.S01E01 (2013), Wanted.AU.2016.S01E01 (2016).
- Show names which are hyphenated or include punctuation must follow the format shown in the title sequence or
  credits of the first episode, limited to the list of acceptable characters.
- If no title card exists, see rule 18.13.1.
- Additional titles and names given to an individual season must not be used.
  e.g. Archer.Vice.S05, Strike.Back.Legacy.S05.
- Acronyms which show the ellipsis of letters with non-standard characters must be replaced with a period.
  e.g. M*A*S\*H must be M.A.S.H.
  e.g. George Carlin... It's Bad for Ya! must be George.Carlin.Its.Bad.For.Ya.
- Directory nomenclature and numbering must remain consistent across the lifetime of an individual show or event.
- Shows which contain acronyms or secondary titles must follow the format used by the first release.
  e.g. Law.and.Order.SVU.S01E01 is the standard format that must be used for all following episodes,
  Law.and.Order.Special.Victims.Unit.S01E02 is not allowed.
  Shadowhunters.The.Mortal.Instruments.S01E01 is the standard format, Shadowhunters.S01E02 is not allowed.
- Shows which air with extended content under modified names must use the primary show name and numbering with
  the EXTENDED tag.
  e.g. QI.S06E01 and QI.XL.S01E01, must be tagged as QI.S06E01 and QI.S06E01.EXTENDED respectively.
  Room.101.S01E01 and Room.101.Extra.Storage.S01E01, must be tagged as Room.101.S01E01 and
  Room.101.S01E01.EXTENDED respectively.
- Groups cannot change the directory format of a show after a second release or episode with the same format
  exists.
  e.g. 2016-01-01: Law.and.Order.SVU.S01E01 sets the format.
  2016-01-08: Law.and.Order.SVU.S01E02 continues the format.
  2016-01-09: Law.and.Order.Special.Victims.Unit.S01E01.DIRFIX is not valid as the second episode already
  exists and continues with the previously defined format.
- Except in situations where the show has an official change in its name, whereby all official references by
  the broadcaster or studio are of the new name. This change must be mentioned in the first NFO with the new
  name with relevant references.
  e.g. Gold.Rush.Alaska.S01E01 changed to Gold.Rush.S02E01.
- Official name changes for a show does not include the renaming of individual seasons. Seasonal name changes
  must be ignored.
  e.g. Power.Rangers.S01 and Power.Rangers.S07 must be used. Power.Rangers.Lost.Galaxy.S07 must not be used.
  Strike.Back.S03, Strike.Back.S05 must be used. Strike.Back.Vengeance.S03, Strike.Back.Legacy.S05 must
  not be used.
