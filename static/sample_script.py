# Title: Sample script
# Description: Sample script to show how to use the templates
# Version: 99.99.99
# Arguments: 
#   - PARAM1
#   - PARAM2
##########################################################################################
# ^
# The format here must be respected, this is so that metadata can be read by the parent CLI. It should be 90 hashtags
# ^
#
# v
# This allows you to list the required arguments and validate them consistently.
# this code should be copy pastes in all scripts, but is not required.
# The shared library is just designed to play nice with the parent CLI
# v
# Using shared lib, import and validate arguments
list = ["PARAM1", "PARAM2"]
from lib.args import args
args = args(list)
##########################################################################################
# From this point onwards, write your script to do whatever you want
print(f"PARAMETER: {args['PARAM1']}, ANOTHER_PARAMETER: {args['PARAM2']}")
exit(0)

