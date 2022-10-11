# Title: Fake process
# Description: Fake script that fakes execution taking {TIME} amount of time
# Version: 99.99.99
# Arguments: 
#   - TIME
##########################################################################################
# Using shared lib, import and validate arguments
list = ["TIME"] 
from lib.args import args
args = args(list)
##########################################################################################
import time
time.sleep(int(args["TIME"]))
print("Wait finished!")
exit(0)
