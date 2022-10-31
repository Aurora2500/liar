export type Player =  {
	name: string;
	dice:DiceSet;
}

export type DiceSet = {
	type: 'unknown';
	length: number;
} | {
	type: 'known';
	dice: Array<number>;
}

export type Bid = {
	quantity: number;
	face: number;
}

export type Variant
	= "raise-quantity"
	| "raise-face"
	| "orthogonal"
	;

export type ServerAction = {
	action: "SetBid",
	bid: Bid
} | {
	action: "SetNick",
}