import { readFileSync } from "fs";
import { JSDOM } from "jsdom";
import { HouseItem } from "./item.js";
import { Area } from "./area.js";
import { Community } from "./community.js";



function item(filepath) {
    let text = readFileSync(filepath).toString();
    let dom = new JSDOM(text);
    let item = new HouseItem(dom.window.document);
    console.log(`id: ${item.getId()}`);
    console.log(`title: ${item.getTitle()}`);
    console.log(`price: ${item.getTotalPrice()}${item.getTotalPriceUnit()}`);
    console.log(`unit price: ${item.getUnitPrice()}${item.getUnitPriceUnit()}`);
    console.log(`room: ${item.getRoom()}`);
    console.log(`layer: ${item.getLayer()}`);
    console.log(`towards: ${item.getToward()}`);
    console.log(`size: ${item.getSize()}`);
    console.log(`build year: ${item.getBuildYear()}`);
    const community = item.getCommunity();
    console.log(`community: ${community.name}(${community.path})`)
    const area = item.getArea()
    console.log(`area: ${area.main.name}(${area.main.path}) - ${area.sub.name}(${area.sub.path})`)
    console.log(`introduce content:`);
    item.getIntroduceContent().forEach(e => {
        console.log(e);
    });
    console.log(`transaction content:`);
    item.getTransaction().forEach(e => {
        console.log(e);
    });
}

function area(filepath) {
    let text = readFileSync(filepath).toString();
    let dom = new JSDOM(text);
    let area = new Area(dom.window.document);
    console.log(`total page: ${area.getPages()}`)

    for (const community of area.getCommunities()) {
        console.log(`id: ${community.id}, name: ${community.name}`);
    }
}

function community(filepath) {
    let text = readFileSync(filepath).toString();
    let dom = new JSDOM(text);
    let community = new Community(dom.window.document);
    console.log(`total page: ${community.getPages()}`)

    for (const house of community.getHouses()) {
        console.log(`id: ${house.id}, name: ${house.name}`);
    }
}

// item("/home/yuki/Projects/resale-houses/.data/dummy/dummy.html")
// area("/home/yuki/Projects/resale-houses/.data/dummy/area.html")
community("/home/yuki/Projects/resale-houses/.data/dummy/community.html")