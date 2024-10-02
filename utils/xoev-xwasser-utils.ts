import type { NatuerlichePersonType } from "pkg/xoev_xwasser"

export const vorname = (person: NatuerlichePersonType) : string | undefined => person.name_natuerliche_person?.vorname?.name
export const familienname = (person: NatuerlichePersonType) : string | undefined => person.name_natuerliche_person?.familienname?.name
