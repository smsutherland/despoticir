use crate::{ChemNetwork, Composition, DustProp, Emitter, Radiation};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Cloud {
    /// number density of H nuclei, in cm^-3
    pub nH: f32,
    /// center-to-edge column density of H nuclei, in cm^-2
    pub colDen: f32,
    /// non-thermal velocity dispersion, in cm s^-1
    pub sigmaNT: f32,
    /// radial velocity gradient, in s^-1 (or cm s^-1 cm^-1)
    pub dVdr: f32,
    /// gas kinetic temperature, in K
    pub Tg: f32,
    /// dust temperature, in K
    pub Td: f32,
    /// a class that stores information about the chemical
    /// composition of the cloud
    pub comp: Composition,
    /// a class that stores information about the properties of the
    /// dust in a cloud
    pub dust: DustProp,
    /// the radiation field impinging on the cloud
    pub rad: Radiation,
    /// keys of the dict are names of emitting species, and values
    /// are objects of class emitter
    pub emitters: HashMap<String, Emitter>,
    /// a chemical network that is to be used to perform
    /// time-dependent chemical evolution calcualtions for this
    /// cloud
    pub chemnetwork: Option<ChemNetwork>,
    /// if set to True, warning messages about convergence of
    /// emitters attached to this cloud are suppressed
    pub noWarn: bool,
}

/// Parameters
///    fileName : string
///       name of file from which to read cloud description
///    noWarn : Boolean
///       if True, warning messages about convergence are
///       suppressed
///    verbose : Boolean
///       print out information about the cloud as we read it
impl Cloud {
    pub fn new(
        file_name: Option<impl AsRef<std::path::Path>>,
        noWarn: bool,
        verbose: bool,
    ) -> Self {
        match file_name {
            None => Self {
                nH: 0.,
                colDen: 0.,
                sigmaNT: 0.,
                dVdr: 0.,
                Tg: 0.,
                Td: 0.,
                comp: Composition,
                dust: DustProp,
                rad: Radiation,
                emitters: HashMap::new(),
                chemnetwork: None,
                noWarn,
            },
            Some(file_name) => {
                let mut res = Self {
                    nH: 0.,
                    colDen: 0.,
                    sigmaNT: 0.,
                    dVdr: 0.,
                    Tg: 0.,
                    Td: 0.,
                    comp: Composition,
                    dust: DustProp,
                    rad: Radiation,
                    emitters: HashMap::new(),
                    chemnetwork: None,
                    noWarn,
                };
                res.read(file_name, verbose);
                res
            }
        }
    }

    /// Read the composition from a file
    ///
    /// Pamameters
    ///    fileName : string
    ///       string giving the name of the composition file
    ///    verbose : Boolean
    ///       print out information about the cloud as it is read
    ///
    /// Returns
    ///    Nothing
    ///
    /// Remarks
    ///    For the format of cloud files, see the documentation
    pub fn read(&mut self, file_name: impl AsRef<std::path::Path>, verbose: bool) {

        // # Read file
        // try:
        //     # First look for the file locally
        //     try:
        //         fp = open(fileName, 'r')
        //     except IOError:
        //         # Look for file in despotic directory
        //         import os.path
        //         # Hack to compute the path to the installed module
        //         # root, so we can load files even when installed
        //         module_dir = os.path.dirname(os.path.realpath(__file__))
        //         fp = open(os.path.join(module_dir, fileName), 'r')
        //         if verbose:
        //             print("Reading from file "+fileName+"...")
        // except IOError:
        //     raise despoticError("cannot open file "+fileName)
        // for line in fp:
        //
        //     # Skip empty and comment lines
        //     if line=='\n':
        //         continue
        //     if line.strip()[0] == "#":
        //         continue
        //
        //     # Break line up based on equal sign
        //     linesplit = line.split("=")
        //     if len(linesplit) < 2:
        //         raise despoticError("Error parsing input line: "+line)
        //     if linesplit[1] == '':
        //         raise despoticError("Error parsing input line: "+line)
        //
        //     # Trim trailing comments from portion after equal sign
        //     linesplit2 = linesplit[1].split('#')
        //
        //     # Proceed based on the token that precedes the equal sign
        //     if linesplit[0].upper().strip() == 'NH':
        //
        //         self.nH = float(linesplit2[0])
        //         if verbose:
        //             print("Setting nH = "+str(self.nH))
        //
        //     elif linesplit[0].upper().strip() == 'COLDEN':
        //
        //         self.colDen = float(linesplit2[0])
        //         if verbose:
        //             print("Setting column density = " +
        //                   str(self.colDen) + " H cm^-2")
        //
        //     elif linesplit[0].upper().strip() == 'SIGMANT':
        //
        //         self.sigmaNT = float(linesplit2[0])
        //         if verbose:
        //             print("Setting sigmaNT = " +
        //                   str(self.sigmaNT) + " cm s^-1")
        //
        //     elif linesplit[0].upper().strip() == 'DVDR':
        //
        //         self.dVdr = float(linesplit2[0])
        //         if verbose:
        //             print("Setting dVdr = " +
        //                   str(self.dVdr) + " cm s^-1 cm^-1")
        //
        //     elif linesplit[0].upper().strip() == 'TG':
        //
        //         self.Tg = float(linesplit2[0])
        //         if verbose:
        //             print("Setting Tg = "+str(self.Tg) + " K")
        //
        //     elif linesplit[0].upper().strip() == 'TD':
        //
        //         self.Td = float(linesplit2[0])
        //         if verbose:
        //             print("Setting Td = "+str(self.Td) + " K")
        //
        //     elif linesplit[0].upper().strip() == 'ALPHAGD':
        //
        //         self.dust.alphaGD = float(linesplit2[0])
        //         if verbose:
        //             print("Setting alpha_GD = " +
        //                   str(self.dust.alphaGD) +
        //                   " erg cm^3 K^-3/2")
        //
        //     elif linesplit[0].upper().strip() == 'SIGMAD10':
        //
        //         self.dust.sigma10 = float(linesplit2[0])
        //         if verbose:
        //             print("Setting sigma_d,10 = " +
        //                   str(self.dust.sigma10) +
        //                   " cm^2 g^-1")
        //
        //     elif linesplit[0].upper().strip() == 'SIGMADPE':
        //
        //         self.dust.sigmaPE = float(linesplit2[0])
        //         if verbose:
        //             print("Setting sigma_d,PE = " +
        //                   str(self.dust.sigmaPE) +
        //                   " cm^2 H^-1")
        //
        //     elif linesplit[0].upper().strip() == 'SIGMADISRF':
        //
        //         self.dust.sigmaISRF = float(linesplit2[0])
        //         if verbose:
        //             print("Setting sigma_d,ISRF = " +
        //                   str(self.dust.sigmaISRF) +
        //                   " cm^2 H^-1")
        //
        //     elif linesplit[0].upper().strip() == 'ZDUST':
        //
        //         self.dust.Zd = float(linesplit2[0])
        //         if verbose:
        //             print("Setting Z'_d = " +
        //                   str(self.dust.Zd))
        //
        //     elif linesplit[0].upper().strip() == 'BETADUST':
        //
        //         self.dust.beta = float(linesplit2[0])
        //         if verbose:
        //             print("Setting beta_dust = "+str(self.dust.beta))
        //
        //     elif linesplit[0].upper().strip() == 'XHI':
        //
        //         self.comp.xHI = float(linesplit2[0])
        //         if verbose:
        //             print("Setting xHI = "+str(self.comp.xHI))
        //
        //     elif linesplit[0].upper().strip() == 'XPH2':
        //
        //         self.comp.xpH2 = float(linesplit2[0])
        //         if verbose:
        //             print("Setting xpH2 = "+str(self.comp.xpH2))
        //
        //     elif linesplit[0].upper().strip() == 'XOH2':
        //
        //         self.comp.xoH2 = float(linesplit2[0])
        //         if verbose:
        //             print("Setting xoH2 = "+str(self.comp.xoH2))
        //
        //     elif linesplit[0].upper().strip() == 'H2OPR':
        //
        //         self.comp.H2OPR = float(linesplit2[0])
        //         if verbose:
        //             print("Setting H2 ortho-para ratio = "+
        //                   str(self.comp.H2OPR))
        //
        //     elif linesplit[0].upper().strip() == 'XH2':
        //
        //         self.comp.xH2 = float(linesplit2[0])
        //         if self.comp.H2OPR is None:
        //             self.comp.H2OPR = 0.25
        //             print("Warning: H2 OPR unspecified, assuming 0.25")
        //         if verbose:
        //             print("Setting xpH2 = "+str(self.comp.xpH2))
        //             print("Setting xoH2 = "+str(self.comp.xoH2))
        //
        //     elif linesplit[0].upper().strip() == 'XHE':
        //
        //         self.comp.xHe = float(linesplit2[0])
        //         if verbose:
        //             print("Setting xHe = "+str(self.comp.xHe))
        //
        //     elif linesplit[0].upper().strip() == 'XE':
        //
        //         self.comp.xe = float(linesplit2[0])
        //         if verbose:
        //             print("Setting xe = "+str(self.comp.xe))
        //
        //     elif linesplit[0].upper().strip() == 'XH+':
        //
        //         self.comp.xe = float(linesplit2[0])
        //         if verbose:
        //             print("Setting xH+ = "+str(self.comp.xe))
        //
        //     elif linesplit[0].upper().strip() == 'TCMB':
        //
        //         self.rad.TCMB = float(linesplit2[0])
        //         if verbose:
        //             print("Setting T_CMB = "+str(self.rad.TCMB)+" K")
        //
        //     elif linesplit[0].upper().strip() == 'TRADDUST':
        //
        //         self.rad.TradDust = float(linesplit2[0])
        //         if verbose:
        //             print("Setting T_radDust = " +
        //                   str(self.rad.TradDust)+" K")
        //
        //     elif linesplit[0].upper().strip() == 'RADDUTDILUTION':
        //
        //         self.rad.fdDilute = float(linesplit2[0])
        //         if verbose:
        //             print("Setting radDust dilution factor = " +
        //                   str(self.rad.fdDilute))
        //
        //     elif linesplit[0].upper().strip() == 'IONRATE':
        //
        //         self.rad.ionRate = float(linesplit2[0])
        //         if verbose:
        //             print("Setting primary ionization rate = " +
        //                   str(self.rad.ionRate)+" s^-1 H^-1")
        //
        //     elif linesplit[0].upper().strip() == 'CHI':
        //
        //         self.rad.chi = float(linesplit2[0])
        //         if verbose:
        //             print("Setting chi = " +
        //                   str(self.rad.chi))
        //
        //     elif linesplit[0].upper().strip() == 'EMITTER':
        //
        //         # Emitter lines are complicated. There are two
        //         # required elements, a name and an abundance, that
        //         # must come first. There are also four optional
        //         # elements: energySkip, noExtrap, file:FileName,
        //         # and URL:url
        //
        //         # Split up the tokens after the equal sign
        //         linesplit3 = linesplit2[0].split()
        //
        //         # Make sure the number of tokens is acceptable
        //         if len(linesplit3) < 2 or len(linesplit3) > 6:
        //             raise despoticError("Error parsing input line: "+line)
        //
        //         # Do we have optional tokens?
        //         if len(linesplit3) == 2:
        //
        //             # Handle case of just two tokens
        //             if verbose:
        //                 print("Adding emitter "+linesplit3[0]+
        //                       " with abundance "+linesplit3[1])
        //             self.addEmitter(linesplit3[0], \
        //                                 float(linesplit3[1]))
        //
        //         else:
        //
        //             # We have optional tokens; initialize various
        //             # options to their defaults, then alter them based
        //             # on the tokens we've been given
        //             energySkip=False
        //             extrap=True
        //             emitterFile=None
        //             emitterURL=None
        //             for token in linesplit3[2:]:
        //                 if token.upper().strip() == 'ENERGYSKIP':
        //                     energySkip=True
        //                 elif token.upper().strip() == 'EXTRAPOLATE':
        //                     # Allowed to maintain backward compatibility
        //                     pass
        //                 elif token.upper().strip() == 'NOEXTRAP':
        //                     extrap=False
        //                 elif token.upper().strip()[0:5] == 'FILE:':
        //                     emitterFile=token[5:].strip()
        //                 elif token.upper().strip()[0:4] == 'URL:':
        //                     emitterURL=token[4:].strip()
        //                 else:
        //                     raise despoticError(
        //                         'unrecognized token "' +
        //                         token.strip()+'" in line: '
        //                         + line)
        //
        //             # Now print message and add emitter
        //             if verbose:
        //                 msg = "Adding emitter "+linesplit3[0]+ \
        //                           " with abundance "+linesplit3[1]
        //                 if energySkip:
        //                     msg += "; setting energySkip"
        //                 if not extrap:
        //                     msg += "; disallowing extrapolation"
        //                 if emitterFile != None:
        //                     msg += "; using file name "+emitterFile
        //                 if emitterURL != None:
        //                     msg += "; using URL "+emitterURL
        //                 print(msg)
        //             self.addEmitter(linesplit3[0],
        //                             float(linesplit3[1]),
        //                             energySkip=energySkip,
        //                             extrap=extrap,
        //                             emitterFile=emitterFile,
        //                             emitterURL=emitterURL)
        //
        //     else:
        //         # Line does not correspond to any known keyword, so
        //         # throw an error
        //         raise despoticError("unrecognized token " +
        //             linesplit[0].strip() + " in file " + fileName)
        //
        // # Close file
        // fp.close()
        //
        // # Check that the hydrogen adds up. If not, raise error
        // if self.comp.xHI + self.comp.xHplus + \
        //         2.0*(self.comp.xpH2 + self.comp.xoH2) != 1:
        //     raise despoticError(
        //         "total hydrogen abundance xHI + xH+ + 2 xH2 != 1")
        //
        // # Set derived properties based on composition, temperature
        // self.comp.computeDerived(self.nH)
        // if self.Tg > 0.0:
        //     self.comp.computeCv(self.Tg)
        //
        // # If verbose, print results for derived quantities
        // if verbose:
        //     print("Derived quantities:")
        //     print("   ===> mean mass per particle = " +
        //           str(self.comp.mu) + " mH")
        //     print("   ===> mean mass per H = " +
        //           str(self.comp.muH) + " mH")
        //     print("   ===> energy added per ionization = " +
        //           str(self.comp.qIon/1.6e-12) + " eV")
        //     if self.Tg > 0.0:
        //         print("   ===> c_v/(k_B n_H mu_H) = " +
        //               str(self.comp.cv))
    }
}
