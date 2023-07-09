import os, time, sys, subprocess

sys.argv.pop(0)
print(sys.argv)
subprocess.Popen(sys.argv, close_fds=True)
