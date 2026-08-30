/**
 * https://www.uprm.edu/p/matricula/abreviaturas_de_los_edificios_rum
 * Abreviatura Edificio
AE Administración de Empresas
AI Antiguo Instituto
AM Ingeniería Agrícola
AP Anexo Piñero
AR Artes Plásticas
AZ Finca Alzamora
B Biología
C Celis
CA Campo Atlético
CD Centro de Investigación y Desarrollo (CID)
CH Chardón
CI Ingeniería Civil
CM Coliseo Mangual
CS Cancha Softball
CT Cancha de Tenis
EA Estudios Aeroespaciales
EE Edificio de Enfermería
Abreviatura Edificio
EI Estación de Isabela
EL Estación de Lajas
EN Laboratorio de Entomología
EP Escuela Pública
F Física, Geología, Ciencias Marinas
FI Otras Fincas
GE Gimnasio Ángel F. Espada
HO Hospital
IA Invernadero de Horticultura
IB‐1
IB‐2
Salones de Instrucción Bibliotecaria
(Biblioteca General)
II Ingeniería Industrial
IC Invernadero de Protección de Cultivos
IH Invernadero de Horticultura
IP Invernadero de Industria Pecuaria
IQ Ingeniería Química
L Antonio Lucchetti (Ingeniería Mecánica)
Abreviaturas de los edificios
en el formulario de matrícula
Abreviatura Edificio
M Luis Monzón (Matemáticas)
MI Miradero (Artes Plásticas)
MG Isla Magueyes
P Jesús T. Piñero (Agricultura)
PA Piscina Alumni
PS Pista Sintética
Q Química
RA Alfredo Ramírez de Arellano y Rosell
S Luis Stefani (Ingeniería)
SA Sánchez Hall (ROTC)
SH Efraín Sánchez Hidalgo (Economía y PPMES)
T Terrats (Pagaduría y Finanzas)
TA Taller de Artes Gráficas
 */

export type RoomData = {
    name: string;
    location?: { lat: number; lng: number };
};

// If a room code does not have a location, it's because I forgor or I don't know it!
export const RoomCodes: Record<string, RoomData> = {
    "AE": { name: "Administración de Empresas", location: { lat: 18.2172817, lng: -67.1424168 } },
    "AI": { name: "Antiguo Instituto" },
    "AM": { name: "Ingeniería Agrícola", location: { lat: 18.2140472, lng: -67.1418556 } },
    "AP": { name: "Anexo Piñero" },
    "AR": { name: "Artes Plásticas", location: { lat: 18.2182359, lng: -67.1431795 } },
    "AZ": { name: "Finca Alzamora", location: { lat: 18.2161113, lng: -67.1464238 } },
    "B": { name: "Biología", location: { lat: 18.2127011, lng: -67.1384620 } },
    "C": { name: "Celis", location: { lat: 18.2094154, lng: -67.1409285 } },
    "CA": { name: "Campo Atlético" },
    "CD": {
        name: "Centro de Investigación y Desarrollo (CID)",
        location: { lat: 18.2114135, lng: -67.1369245 },
    },
    "CH": { name: "Chardón", location: { lat: 18.2106352, lng: -67.1402402 } },
    "CI": { name: "Ingeniería Civil", location: { lat: 18.2148355, lng: -67.1392380 } },
    "CM": { name: "Coliseo Mangual", location: { lat: 18.2130325, lng: -67.1439427 } },
    "CS": { name: "Cancha Softball" },
    "CT": { name: "Cancha de Tenis", location: { lat: 18.216166, lng: -67.1448617 } },
    "EA": { name: "Estudios Aeroespaciales", location: { lat: 18.2086949, lng: -67.1401114 } },
    "EE": { name: "Edificio de Enfermería", location: { lat: 18.2123903, lng: -67.1418982 } },
    "EI": { name: "Estación de Isabela" },
    "EL": { name: "Estación de Lajas" },
    "EN": { name: "Laboratorio de Entomología" },
    "EP": { name: "Escuela Pública" },
    "F": { name: "Física, Geología, Ciencias Marinas", location: { lat: 18.2110242, lng: -67.1391462 } },
    "FI": { name: "Otras Fincas" },
    "GE": { name: "Gimnasio Ángel F. Espada", location: { lat: 18.2120494, lng: -67.1432023 } },
    "HO": { name: "Hospital", location: { lat: 18.2101599, lng: -67.1426064 } },
    "IA": { name: "Invernadero de Horticultura" },
    "IB-1": {
        name: "Salones de Instrucción Bibliotecaria (Biblioteca General)",
        location: { lat: 18.2112289, lng: -67.1417261 },
    },
    "IB-2": {
        name: "Salones de Instrucción Bibliotecaria (Biblioteca General)",
        location: { lat: 18.2112289, lng: -67.1417261 },
    },
    "II": { name: "Ingeniería Industrial", location: { lat: 18.2105104, lng: -67.1396452 } },
    "IC": { name: "Invernadero de Protección de Cultivos" },
    "IH": { name: "Invernadero de Horticultura" },
    "IP": { name: "Invernadero de Industria Pecuaria" },
    "IQ": { name: "Ingeniería Química", location: { lat: 18.2147517, lng: -67.1402723 } },
    "L": { name: "Antonio Lucchetti (Ingeniería Mecánica)", location: { lat: 18.2086949, lng: -67.1401114 } },
    "M": { name: "Luis Monzón (Matemáticas)", location: { lat: 18.2097013, lng: -67.1417944 } },
    "MI": { name: "Miradero (Artes Plásticas)", location: { lat: 18.2168371, lng: -67.1409592 } },
    "MG": { name: "Isla Magueyes" },
    "P": { name: "Jesús T. Piñero (Agricultura)", location: { lat: 18.2103712, lng: -67.1437366 } },
    "PA": { name: "Piscina Alumni", location: { lat: 18.2162657, lng: -67.1434588 } },
    "PS": { name: "Pista Sintética", location: { lat: 18.214371, lng: -67.1436449 } },
    "Q": { name: "Química", location: { lat: 18.2126203, lng: -67.1407494 } },
    "RA": { name: "Alfredo Ramírez de Arellano y Rosell" },
    "S": { name: "Luis Stefani (Ingeniería)", location: { lat: 18.2097509, lng: -67.1393112 } },
    "SA": { name: "Sánchez Hall (ROTC)", location: { lat: 18.2091277, lng: -67.1398739 } },
    "SH": {
        name: "Efraín Sánchez Hidalgo (Economía y PPMES)",
        location: { lat: 18.211705141192773, lng: -67.14033669830644 },
    },
    "OF": { name: "Oficinas de Facultad", location: { lat: 18.2117534, lng: -67.141015 } },
    "T": { name: "Terrats (Pagaduría y Finanzas)", location: { lat: 18.2103244, lng: -67.1392446 } },
    "TA": { name: "Taller de Artes Gráficas", location: { lat: 18.2133836, lng: -67.1413917 } },
};
