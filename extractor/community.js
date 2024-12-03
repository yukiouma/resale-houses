export class Community {
    /**
     * @param {Document} doc 
     */
    constructor(doc) {
        this.doc = doc;
    }

    getPages() {
        const node = this.doc.querySelector(".page-box.house-lst-page-box");
        if (node) {
            return parseInt(JSON.parse(node.getAttribute("page-data")).totalPage);
        }
        return 0;
    }

    getHouses() {
        const nodes = this.doc.querySelectorAll(".sellListContent .img.maidian-detail");
        const result = [];
        nodes.forEach(node => {
            const name = node.getAttribute("title");
            const id = node.getAttribute("href").split("/")[4].replace(".html", "");
            result.push({ name, id });
        });
        return result;
    }
}