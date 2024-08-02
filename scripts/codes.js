import { parseXml } from 'libxmljs2'
import camelCase from 'camelcase';
import { readFileSync } from 'fs'

const files = [
    "schemas/remote/0fd6e3cbcdf3573e93d05085f352b4ac",
    "schemas/V0_5_0/xwasser-basisnachricht.xsd",
    "schemas/V0_5_0/xwasser-baukasten-erweiterung.xsd",
    "schemas/V0_5_0/xwasser-baukasten.xsd",
    "schemas/V0_5_0/xwasser-rueckweisung.xsd",
]

for (const file of files) {
    const source = readFileSync(file, 'utf-8');
    const xml = parseXml(source);
    for (const node of xml.find('/xs:schema/xs:complexType', { xs: 'http://www.w3.org/2001/XMLSchema' })) {
        const name = node.attr("name")?.value();
        if (name) {
            if (name.startsWith("Code.")) {
                const structName = camelCase(name, { pascalCase: true });
                const desc = node.get('xs:annotation/xs:appinfo/codeliste/beschreibung', { xs: 'http://www.w3.org/2001/XMLSchema' })?.text();
                const urn = node.get('xs:annotation/xs:appinfo/codeliste/kennung', { xs: 'http://www.w3.org/2001/XMLSchema' })?.text();
                const version = node.get('xs:annotation/xs:appinfo/versionCodeliste/version', { xs: 'http://www.w3.org/2001/XMLSchema' })?.text();
                if (desc && urn) {
                    if (version) {
                        console.log(`
/// Type name: ${name}
/// ${desc}
#[xoev_xwasser_code("${urn}", "${version}")]
pub struct ${structName};`);
                    } else {
                        console.log(`
/// Type name: ${name}
/// ${desc}
#[xoev_xwasser_code("${urn}")]
pub struct ${structName};`);
                    }
                }
            }
        }
    }
}
