# TEC GTFS Enhancer

Small tool to combine BMC and LETEC GTFS feeds to get benefits from both

**The LETEC feed is used as base**

## Modifications (to the tec feed)

- Add route_text_color automatically from route_color
- Modify feed_info with repository URL and new version
- Import trip headsigns from bmc (they're equivalent to the real instead (usually last stop name))
- Import stops wheelchair boarding info from BMC
