import json
import sys
import numpy as np
from scipy import signal as sp
import matplotlib.pyplot as plt
from pathlib import Path, PureWindowsPath, WindowsPath
import multiprocessing as mp

def serie_single_processor(serie):
    id_datalist = list( filter((lambda dat: dat['designator'] == 'Id'),serie['data']) ) 
    t_datalist = list( filter((lambda dat: dat['designator'] == 'T(s)'),serie['data']) )
    id_arr = np.array(id_datalist[0]['data'][0])
    t_arr = np.array(t_datalist[0]['data'][0])

    fs = 1/np.average(np.diff(t_arr))
    
    f, psd = sp.periodogram(id_arr,fs,return_onesided = True)
    logpsd = 10*np.log10(psd)
    plt.semilogx(f[2:],logpsd[2:])
    plt.title("Power Spectral Density of: "+serie['title'])
    plt.autoscale('both',tight=True)
    plt.ylabel('PSD(V^2/Hz)')
    plt.xlabel('f(Hz)')

    outputdir = PureWindowsPath(sys.argv[2]) 
    filename = Path(outputdir/PureWindowsPath("psd\\" + serie['title'] + ".png"))
    plt.savefig(filename,dpi=600)
    plt.close()

plt.ioff()
scriptdir = sys.argv[1]
scriptdir = PureWindowsPath(scriptdir)

outputdir = sys.argv[2]
outputdir = PureWindowsPath(outputdir + '\\psd')
if not Path(outputdir).is_dir():
    Path(outputdir).mkdir()

filename = Path(scriptdir/PureWindowsPath('data\\psd.json'))
with open(filename,encoding="utf-8") as json_file:
    data = json.load(json_file)
    if __name__ == '__main__':
        with mp.Pool(processes=mp.cpu_count())  as p:
            p.map(serie_single_processor, data)
        