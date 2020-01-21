import json
import sys
import numpy as np
from scipy import signal as sp
import matplotlib.pyplot as plt
from pathlib import Path, PureWindowsPath, WindowsPath

plt.ioff()
scriptdir = sys.argv[1]
scriptdir = PureWindowsPath(scriptdir)
outputdir = sys.argv[2]
outputdir = PureWindowsPath(outputdir)
filename = Path(scriptdir/PureWindowsPath('psd.json'))
with open(filename,encoding="utf-8") as json_file:
    data = json.load(json_file)
    for serie in data:
        id_datalist = list( filter((lambda dat: dat['designator'] == 'Id'),serie['data']) ) 
        t_datalist = list( filter((lambda dat: dat['designator'] == 'T(s)'),serie['data']) )
        id_arr = np.array(id_datalist[0]['data'][0])
        t_arr = np.array(t_datalist[0]['data'][0])

        fs = 1/np.average(np.diff(t_arr))
        
        f, psd = sp.periodogram(id_arr,fs,return_onesided = True)
        logpsd = 10*np.log10(psd)
        sys.stderr.write(str(logpsd[2:]) + '\n')
        #logpsd[logpsd<-200] = -200
        plt.semilogx(f[2:],logpsd[2:])
        plt.title("Power Spectral Density of: "+serie['title']) 
        filename = Path(outputdir/PureWindowsPath("psd " + serie['title'] + ".png"))
        sys.stderr.write(str(filename) + '\n')
        plt.autoscale('both',tight=True)
        plt.savefig(filename,dpi=600)
        plt.close()