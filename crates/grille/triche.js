let n = [];
let j = [];
let v = [];
let n2 = [];
let j2 = [];
let v2 = [];
let rangées = [];
let tuiles = document.querySelectorAll("div[data-state]");

// -n -j -v
let rangée = [];
for (let i = 0; i < tuiles.length; i++) {
    let label = tuiles[i].getAttribute("aria-label");
    if (label == "empty") {
        break;
    } else {
        rangée.push(label);
        if (i % 5 == 4) {
            rangées.push(rangée);
            rangée = [];
        }
        let split = label.split(' ');
        if (split[1] == "correct") {
            v.push(split[0] + ((i % 5) + 1));
        } else if (split[1] == "present") {
            j.push(split[0] + ((i % 5) + 1));
        } else {
            n.push(split[0]);
        }
    }
}

// -N
for (let rangée of rangées) {
    let i = 0;
    for (let label of rangée) {
        let split = label.split(' ');
        if (split[1] == "absent") {
            let noire = split[0];
            let pos = i;
            let jaune = new Array(5).fill(false);
            let verte = new Array(5).fill(false);
            let j = 0;
            for (let label of rangée) {
                let split = label.split(' ');
                if (split[0] == noire && j != pos) {
                    if (split[1] == "present") {
                        jaune[j] = true;
                    } else {
                        verte[j] = true;
                    }
                }
                j++;
            }
            if (jaune.some((b) => b)) {
                n2.push(noire + (i + 1));
            } else if (verte.some((b) => b)) {
                for (let i = 0; i < 5; i++) {
                    if (!verte[i]) {
                        n2.push(noire + (i + 1));
                    }
                }
            }
        }
        i++;
    }
}

console.log(n);
console.log(j);
console.log(v);
console.log(n2);
