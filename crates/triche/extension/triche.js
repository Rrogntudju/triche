function triche() {
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
        let label = tuiles[i].getAttribute("data-state");
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

    // Combinaisons de 2 lettres identiques sur la même ligne (exemples avec BLEEP)
    // noire + verte : la lettre est unique et n'est présente qu'à la position de la lettre verte. -N b2 b3 b4 b5 -v b1
    // noire + jaune : la lettre est unique et présente ailleurs que la lettre jaune ou noire. -N b2 -j b3
    // jaune + jaune : les 2 lettres sont présentes dans les positions restantes. -J e1 e5
    // verte + jaune : les 2 lettres sont présentes mais la jaune se trouve ailleurs que la verte. -V e3 e5

    // -N
    for (let rangée of rangées) {
        let i = 0;
        for (let label of rangée) {
            let split = label.split(' ');
            if (split[1] == "absent") {
                let noire = split[0];
                let jaune = new Array(5).fill(false);
                let verte = new Array(5).fill(false);
                let j = 0;
                for (let label of rangée) {
                    let split = label.split(' ');
                    if (split[0] == noire && j != i) {
                        if (split[1] == "present") {
                            jaune[j] = true;
                        } else if (split[1] == "correct") {
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

    // -J
    J: {
        for (let rangée of rangées) {
            let i = 0;
            for (let label of rangée) {
                let split = label.split(' ');
                if (split[1] == "present") {
                    let jaune = split[0];
                    let j = 0;
                    for (let label of rangée) {
                        let split = label.split(' ');
                        if (split[1] == "present" && split[0] == jaune && j != i) {
                            j2.push(jaune + (i + 1));
                            j2.push(split[0] + (j + 1));
                            break J;
                        }
                        j++;
                    }
                }
                i++;
            }
        }
    }

    // -V
    V: {
        for (let rangée of rangées) {
            let i = 0;
            for (let label of rangée) {
                let split = label.split(' ');
                if (split[1] == "correct") {
                    let verte = split[0];
                    let j = 0;
                    for (let label of rangée) {
                        let split = label.split(' ');
                        if (split[1] == "present" && split[0] == verte) {
                            v2.push(verte + (i + 1));
                            v2.push(split[0] + (j + 1));
                            break V;
                        }
                        j++;
                    }
                }
                i++;
            }
        }
    }

    let commande = "triche";
    if (n.length != 0) {
        commande += " -n " + n.join(' ');
    }
    if (j.length != 0) {
        commande += " -j " + j.join(' ');
    }
    if (v.length != 0) {
        commande += " -v " + v.join(' ');
    }
    if (n2.length != 0) {
        commande += " -N " + n2.join(' ');
    }
    if (j2.length != 0) {
        commande += " -J " + j2.join(' ');
    }
    if (v2.length != 0) {
        commande += " -V " + v2.join(' ');
    }

    console.log(commande);
    navigator.clipboard.writeText(commande)
    .catch((err) => console.log(err));
}

var interval = setInterval(function() {
    var container = document.getElementById('wordle-app-game');
    if (container) {
        var btn = document.createElement('button');
        btn.textContent = 'triche';
        btn.addEventListener("click", triche);
        container.appendChild(btn);
        clearInterval(interval);
    }
}, 250);

