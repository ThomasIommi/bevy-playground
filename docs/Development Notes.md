# **ECS – Entity Component System**

**ECS** è un pattern architetturale utilizzato nello sviluppo di videogiochi per la gestione degli oggetti di gioco. Promuove un approccio **[[Data-Oriented Programming|data-oriented]]** e **decoupled**, separando chiaramente i dati dalla logica.
## **Concetti fondamentali**

### Entity
- È un **identificativo univoco** (ID), privo di dati o logica.
- Rappresenta un oggetto di gioco astratto, come un **giocatore**, un **nemico** o un **proiettile**.
- Può essere inteso come un "contenitore vuoto" o un riferimento a cui vengono associati dei componenti.
### Component
- È una **struttura dati pura**, senza logica.
- Ogni componente rappresenta un **aspetto specifico** dell'entità, ad esempio:
    - `PositionComponent`
    - `HealthComponent`
    - `VelocityComponent`
- I componenti vengono **associati a una entity** per dotarla di tratti specifici (composizione).
### System
- Contiene la **logica di gioco**.
- I sistemi processano le entity che possiedono un determinato set di componenti.
- Sono responsabili di **modificare i valori** dei componenti in base alla logica, ad esempio:
    - Un `MovementSystem` modifica `PositionComponent` in base a `VelocityComponent`.
## Vantaggi

- **Performance migliorata** grazie a un layout di memoria più efficiente e parallelizzabile.
- **Flessibilità**: i comportamenti degli oggetti sono definiti dai componenti associati.
- **Separazione delle responsabilità**: dati e logica sono nettamente separati.
## **Tag**

#ecs #game-dev