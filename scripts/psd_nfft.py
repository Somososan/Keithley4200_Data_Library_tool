import json
import sys
import math
import numpy as np
from scipy import signal as sp
import matplotlib.pyplot as plt
from pathlib import Path, PureWindowsPath, WindowsPath
import multiprocessing as mp
import nfft
import warnings
#warnings.filterwarnings('ignore')

def strip_non_ascii(string):
    ''' Returns the string without non ASCII characters'''
    stripped = (c for c in string if 0 < ord(c) < 127)
    return ''.join(stripped)

def serie_single_processor(serie):
    id_datalist = list( filter((lambda dat: dat['designator'] == 'Id'),serie['data']) ) 
    t_datalist = list( filter((lambda dat: dat['designator'] == 'T(s)'),serie['data']) )
    id_arr = np.array(id_datalist[0]['data'][0])
    t_arr = np.array(t_datalist[0]['data'][0])
    if (len(id_arr) % 2) != 0:
        sys.stdout.writelines(strip_non_ascii( serie['title']) + '\n\n')
        sys.stdout.flush()
    fs = 1/np.average(np.diff(t_arr))
    
    N=len(t_arr)
    if N %2 ==0:
        dft_two_sided = np.real(nfft.nfft(np.arange(-0.5,0.5,1/N),id_arr))
    else:
        dft_two_sided = np.real(nfft.ndft(t_arr,id_arr))
    dft_one_sided = 2*dft_two_sided[1:math.ceil(N/2)]
    psd = (1/(2*math.pi*N)) * np.square(dft_one_sided)
    f = np.arange(0,math.pi*fs,(2*math.pi*fs)/N )[1:]
    logpsd = 10*np.log10(psd)
    plt.semilogx(f,logpsd)
    plt.title("Power Spectral Density of: "+serie['title'])
    plt.autoscale('both',tight=True)
    plt.ylabel('PSD(A^2/Hz)')
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
        