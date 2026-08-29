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

export const RoomCodes: Record<string, RoomData> = {
    "AE": { name: "Administración de Empresas" },
    "AI": { name: "Antiguo Instituto" },
    "AM": { name: "Ingeniería Agrícola" },
    "AP": { name: "Anexo Piñero" },
    "AR": { name: "Artes Plásticas" },
    "AZ": { name: "Finca Alzamora" },
    "B": { name: "Biología" },
    "C": { name: "Celis" },
    "CA": { name: "Campo Atlético" },
    "CD": { name: "Centro de Investigación y Desarrollo (CID)" },
    "CH": { name: "Chardón" },
    "CI": { name: "Ingeniería Civil" },
    "CM": { name: "Coliseo Mangual" },
    "CS": { name: "Cancha Softball" },
    "CT": { name: "Cancha de Tenis" },
    "EA": { name: "Estudios Aeroespaciales" },
    "EE": { name: "Edificio de Enfermería" },
    "EI": { name: "Estación de Isabela" },
    "EL": { name: "Estación de Lajas" },
    "EN": { name: "Laboratorio de Entomología" },
    "EP": { name: "Escuela Pública" },
    "F": { name: "Física, Geología, Ciencias Marinas" },
    "FI": { name: "Otras Fincas" },
    "GE": { name: "Gimnasio Ángel F. Espada" },
    "HO": { name: "Hospital" },
    "IA": { name: "Invernadero de Horticultura" },
    "IB-1": {
        name: "Salones de Instrucción Bibliotecaria (Biblioteca General)",
    },
    "IB-2": {
        name: "Salones de Instrucción Bibliotecaria (Biblioteca General)",
    },
    "II": { name: "Ingeniería Industrial" },
    "IC": { name: "Invernadero de Protección de Cultivos" },
    "IH": { name: "Invernadero de Horticultura" },
    "IP": { name: "Invernadero de Industria Pecuaria" },
    "IQ": { name: "Ingeniería Química" },
    "L": { name: "Antonio Lucchetti (Ingeniería Mecánica)" },
    "M": { name: "Luis Monzón (Matemáticas)" },
    "MI": { name: "Miradero (Artes Plásticas)" },
    "MG": { name: "Isla Magueyes" },
    "P": { name: "Jesús T. Piñero (Agricultura)" },
    "PA": { name: "Piscina Alumni" },
    "PS": { name: "Pista Sintética" },
    "Q": { name: "Química" },
    "RA": { name: "Alfredo Ramírez de Arellano y Rosell" },
    "S": { name: "Luis Stefani (Ingeniería)" },
    "SA": { name: "Sánchez Hall (ROTC)" },
    "SH": {
        name: "Efraín Sánchez Hidalgo (Economía y PPMES)",
        location: { lat: 18.211705141192773, lng: -67.14033669830644 },
    },
    "T": { name: "Terrats (Pagaduría y Finanzas)" },
    "TA": { name: "Taller de Artes Gráficas" },
};
