# netlab-cli

📡 **netlab-cli** è un tool da riga di comando scritto in Rust pensato per studenti e appassionati di reti.
Permette di eseguire calcoli ed esercizi tipici del programma di **Reti e Calcolo Subnetting** (IPv4, FLSM, CIDR, ecc.).

---

## ✨ Funzionalità
- Calcolo host necessari e host sprecati
- Numero di subnet disponibili (FLSM)
- Calcolo host per subnet
- Conversione prefix ↔ netmask / wildcard
- Net-ID, Broadcast e Gateway di una rete
- Selezione host specifici all’interno di una subnet
- Controllo appartenenza IP a subnet
- Menu interattivo in modalità CLI

---

## 🚀 Installazione

Clona la repo e compila con Cargo:

```bash
git clone https://github.com/tuo-utente/netlab-cli.git
cd netlab-cli
cargo build --release
```


## TODO
- Wildcard da prefix

- Ultimo host disponibile

- Controllo appartenenza IP a subnet

- CIDR supernetting => non penso devo prima capire

- Supporto IPv6
