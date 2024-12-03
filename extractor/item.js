export class HouseItem {
    /**
     * init item instance
     * @param {Document} doc 
     */
    constructor(doc) {
        this.doc = doc;
    }

    getTitle() {
        return this.doc.getElementsByClassName("main").item(0).textContent.trim();
    }

    getTotalPrice() {
        return this.doc.getElementsByClassName("total").item(0).textContent.trim();
    }

    getTotalPriceUnit() {
        return this.doc.getElementsByClassName("unit").item(0).children.item(0).textContent.trim();
    }

    getUnitPrice() {
        return this.doc.getElementsByClassName("unitPrice").item(0).children.item(0).textContent.trim();
    }

    getUnitPriceUnit() {
        return this.doc.getElementsByClassName("unitPrice").item(0).children.item(1).textContent.trim();
    }

    getRoom() {
        return this.doc.querySelector(".room .mainInfo").textContent.trim();
    }

    getLayer() {
        return this.doc.querySelector(".room .subInfo").textContent.trim();
    }

    getToward() {
        return this.doc.querySelector(".type .mainInfo").textContent.trim();
    }

    getSize() {
        return this.doc.querySelector(".area .mainInfo").textContent.trim();
    }

    getBuildYear() {
        return this.doc.querySelector(".area .subInfo").textContent.trim();
    }

    getId() {
        return this.doc.querySelector(".houseRecord .info").textContent.trim().split("\n")[0].trim();
    }

    getCommunity() {
        const node = this.doc.querySelector(".communityName a.info");
        return {
            path: node.href,
            name: node.textContent.trim(),
        }
    }

    getArea() {
        const nodes = this.doc.querySelectorAll(".areaName a");
        return {
            main: {
                name: nodes[0].textContent.trim(),
                path: nodes[0].href,
            },
            sub: {
                name: nodes[1].textContent.trim(),
                path: nodes[1].href,
            },
        }
    }

    getIntroduceContent() {
        const content = this.doc.querySelector(".introContent .base .content").children.item(0).children;
        return Array.from(content).map(e => {
            let lines = e.textContent.trim().split("\n");
            return {
                key: lines[0] ? lines[0].trim() : "NA",
                value: lines[1] ? lines[1].trim() : "NA",
            };
        })

    }

    getTransaction() {
        const content = this.doc.querySelector(".introContent .transaction .content").children.item(0).children;
        return Array.from(content).map(e => {
            let lines = e.textContent.trim();
            return {
                key: lines.slice(0, 4),
                value: lines.slice(4),
            };
        })

    }
}