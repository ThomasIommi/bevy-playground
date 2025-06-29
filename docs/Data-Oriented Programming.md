# DOP - Data-Oriented Programming

**Data-Oriented Programming (DOP)** è un paradigma di programmazione che si focalizza sull'organizzazione efficiente dei **dati in memoria** per ottenere **massime prestazioni**, specialmente in contesti ad alta intensità computazionale (es. videogiochi, simulazioni, rendering).

A differenza della **OOP** (Object-Oriented Programming), dove i dati e il comportamento sono strettamente legati negli oggetti, in DOP i **dati e le funzioni sono separati**. Questo avvicina il paradigma al **Functional Programming (FP)** sotto alcuni aspetti, ma con obiettivi diversi: FP mira alla **manutenibilità e testabilità**, mentre DOP si concentra su **performance e cache efficiency**.
## Concetti fondamentali

### Dati come strutture piatte
- Le strutture dati devono essere **semplici e prevedibili**, spesso composte solo da tipi primitivi o strutture non annidate.
- Preferiti layout in **Struct of Arrays (SoA)** rispetto ad **Array of Structs (AoS)**.
### Separazione tra dati e comportamento
- I **dati** non contengono metodi.
- Le **funzioni** che agiscono sui dati sono separate, spesso raggruppate in **sistemi** o **moduli** che processano insiemi di dati.
### Accesso alla memoria prevedibile
- I dati sono memorizzati in **blocchi contigui** per migliorare la **cache locality**.
- Le operazioni sono progettate per ridurre i cache miss, branch misprediction, e accessi disordinati.
### Elaborazione in batch
- Si processano **interi insiemi di entità** in loop lineari o in parallelo, migliorando il throughput.
- Ottimale per SIMD, thread multipli e GPU.
- Parallelismo facilitato.

## **Tag**

#programming

