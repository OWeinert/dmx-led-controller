# Remaining Issues and possible fixes

## Not all dmx signals get decoded correctly

The DMX protocol doesn't specify all timings exactly. Lots of DMX Controllers have different timings for the break and mark after break. Currently, many Controllers are therefore not yet recognized.

libsigrokdecode most recent version v.0.5.3 unfortunate doesn't support [dmx512 options](https://github.com/sigrokproject/libsigrokdecode/blob/libsigrokdecode-0.5.3/decoders/dmx512/pd.py).
 The master branchs version tho supports a varaitey of [configuratoins](
https://github.com/sigrokproject/libsigrokdecode/blob/master/decoders/dmx512/pd.py).

So building the library's master version from scratch, could solve that problem.

