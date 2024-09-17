# Make sure an instance of the API is running
PARKOUR_API_SAVE_TIMER=1 PARKOUR_API_SECRET=your_secret_here cargo run


#
#  ███████╗██╗   ██╗███████╗███╗   ██╗████████╗███████╗
#  ██╔════╝██║   ██║██╔════╝████╗  ██║╚══██╔══╝██╔════╝
#  █████╗  ██║   ██║█████╗  ██╔██╗ ██║   ██║   ███████╗
#  ██╔══╝  ╚██╗ ██╔╝██╔══╝  ██║╚██╗██║   ██║   ╚════██║
#  ███████╗ ╚████╔╝ ███████╗██║ ╚████║   ██║   ███████║
#  ╚══════╝  ╚═══╝  ╚══════╝╚═╝  ╚═══╝   ╚═╝   ╚══════╝
#

# Get the list of events
curl http://localhost:3030/v1/events -H "authentication: your_secret_here"

# Create event
curl -X POST http://localhost:3030/v1/events -H "authentication: your_secret_here" -H "Content-Type: application/json" --data @docs/example/body/event.json


#
#  ███╗   ███╗ █████╗ ██████╗ ███████╗
#  ████╗ ████║██╔══██╗██╔══██╗██╔════╝
#  ██╔████╔██║███████║██████╔╝███████╗
#  ██║╚██╔╝██║██╔══██║██╔═══╝ ╚════██║
#  ██║ ╚═╝ ██║██║  ██║██║     ███████║
#  ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝     ╚══════╝
#

# Get the list of maps
curl http://localhost:3030/v1/events/:event_id/maps -H "authentication: your_secret_here"

# Create map
curl -X POST http://localhost:3030/v1/events/:event_id/maps -H "authentication: your_secret_here" -H "Content-Type: application/json" --data @docs/example/body/map.json


#
#   ██████╗ ██████╗ ███╗   ██╗███████╗██╗ ██████╗ 
#  ██╔════╝██╔═══██╗████╗  ██║██╔════╝██║██╔════╝ 
#  ██║     ██║   ██║██╔██╗ ██║█████╗  ██║██║  ███╗
#  ██║     ██║   ██║██║╚██╗██║██╔══╝  ██║██║   ██║
#  ╚██████╗╚██████╔╝██║ ╚████║██║     ██║╚██████╔╝
#   ╚═════╝ ╚═════╝ ╚═╝  ╚═══╝╚═╝     ╚═╝ ╚═════╝ 
#

# Get map configurations
curl http://localhost:3030/v1/maps/:map_id/configurations -H "authentication: your_secret_here"

# Create map configuration
curl -X POST http://localhost:3030/v1/maps/:map_id/configurations -H "authentication: your_secret_here" -H "Content-Type: application/json" --data @docs/example/body/config.json


#
#  ███████╗ ██████╗ ██████╗ ██████╗ ███████╗███████╗
#  ██╔════╝██╔════╝██╔═══██╗██╔══██╗██╔════╝██╔════╝
#  ███████╗██║     ██║   ██║██████╔╝█████╗  ███████╗
#  ╚════██║██║     ██║   ██║██╔══██╗██╔══╝  ╚════██║
#  ███████║╚██████╗╚██████╔╝██║  ██║███████╗███████║
#  ╚══════╝ ╚═════╝ ╚═════╝ ╚═╝  ╚═╝╚══════╝╚══════╝
#

# Get the scores
curl http://localhost:3030/v1/configurations/:config_id/scores -H "authentication: your_secret_here"

# Submit a new score
curl -X POST http://localhost:3030/v1/configurations/:config_id/scores -H "authentication: your_secret_here" -H "Content-Type: application/json" --data @docs/example/body/score.json
