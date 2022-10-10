# Contains python scripts to validate arguments 
# 
############################################################################################
from argparse import ArgumentParser

def args(list_arguments):
    """
    Using list_arguments provided, returns dictionary of validated args.

    Parameters
    ----------
    list_arguments : list
        Arguments to validate.
    """
    # Parse arguments into a dictionary
    parser = ArgumentParser()
    for arg in list_arguments:
        parser.add_argument("-"+arg)
    dict = {}
    parsed = vars(parser.parse_args())
    for p_arg in parsed:
        dict[p_arg] = parsed[p_arg]
    failed = False
    # Validate dictionary
    for arg in list_arguments:
        if parsed[arg] == None:
            print(f"{arg} was not supplied")
            failed = True
    if failed:
        print("FATAL: could not parse arguments")
        exit(1)
    else:
        return dict
