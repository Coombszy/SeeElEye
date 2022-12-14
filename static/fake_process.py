# Title: Fake process
# Description: Fake script that fakes execution taking {FAKE_TIME} amount of time
# Version: 99.99.99
# Arguments: 
#   - FAKE_TIME
##########################################################################################
# Using shared lib, import and validate arguments
list = ["FAKE_TIME"]
from lib.args import args
args = args(list)
##########################################################################################
import time
print(f"Waiting for {args['FAKE_TIME']} seconds... ", end='')
time.sleep(float(args["FAKE_TIME"]))
print("Wait finished!")
exit(0)
