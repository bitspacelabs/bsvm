import {
    initEccLib,
    networks,
    script as Script,
    Signer,
    payments,
    crypto,
    Psbt
  } from "bitcoinjs-lib";
import { ECPairFactory, ECPairAPI, TinySecp256k1Interface } from "ecpair";
  
  const tinysecp: TinySecp256k1Interface = require("tiny-secp256k1");
  initEccLib(tinysecp as any);
  const ECPair: ECPairAPI = ECPairFactory(tinysecp);

call_space({
    // spaceCode: `btc-vm-space-token-0 transfer [100]`,
    spaceCode: `btc-vm-space-token-0 mint`,
    WIF: '',
    fee_value: 1000,
    utxo: {"txid":"xxxx","vout":1,"value": 10000}
})

async function call_space  ({
    spaceCode,
    WIF,
    recv,
    fee_value,
    utxo,
  }: {
    spaceCode: string;
    WIF: string;
    is_send?: boolean;
    is_decode?: boolean;
    recv?: string;
    fee_value: number;
    utxo?: IUTXO,
  }) {
    const network1 = networks.bitcoin

    const keypair = ECPair.fromWIF(WIF, network1);
    const tweakedSigner = tweakSigner(keypair, { network: network1 });
    const p2pktr = payments.p2tr({
      pubkey: toXOnly(tweakedSigner.publicKey),
      network: network1,
    });
  
    const payerAddress = p2pktr.address;
    if (!payerAddress) {
      return console.log("payerAddress error");
    }

    console.log('payerAddress::', payerAddress)
  
    const psbt = new Psbt({ network: network1 });
    psbt.addInput({
      hash: utxo.txid,
      index: utxo.vout,
      witnessUtxo: { value: utxo.value, script: p2pktr.output! },
      tapInternalKey: toXOnly(keypair.publicKey),
    });
  
    const opReturnOutput = encode_issue('S', spaceCode);

    psbt.addOutput({
      script: opReturnOutput!,
      value: 0,
    });
    let recv_value = 0
    if(recv){
      psbt.addOutput({
        address: recv || p2pktr.address!,
        value: 330,
      });
      recv_value = 330
    }
    psbt.addOutput({
      address: p2pktr.address!,
      value: utxo.value - recv_value - fee_value,
    });
  
    psbt.signInput(0, tweakedSigner);
    psbt.finalizeAllInputs();
  
    const tx = psbt.extractTransaction();
    console.log(`Broadcasting Transaction Hex: ${tx.toHex()}`);
  };
  
  
  function tweakSigner(signer: Signer, opts: any = {}): Signer {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    let privateKey: Uint8Array | undefined = signer.privateKey!;
    if (!privateKey) {
      throw new Error("Private key is required for tweaking signer!");
    }
    if (signer.publicKey[0] === 3) {
      privateKey = tinysecp.privateNegate(privateKey);
    }
  
    const tweakedPrivateKey = tinysecp.privateAdd(
      privateKey,
      tapTweakHash(toXOnly(signer.publicKey), opts.tweakHash)
    );
    if (!tweakedPrivateKey) {
      throw new Error("Invalid tweaked private key!");
    }
  
    return ECPair.fromPrivateKey(Buffer.from(tweakedPrivateKey), {
      network: opts.network,
    });
  }
  
  function tapTweakHash(pubKey: Buffer, h: Buffer | undefined): Buffer {
    return crypto.taggedHash(
      "TapTweak",
      Buffer.concat(h ? [pubKey, h] : [pubKey])
    );
  }
  
  function toXOnly(pubkey: Buffer): Buffer {
    return pubkey.subarray(1, 33);
  }


  interface IUTXO {
    txid: string;
    vout: number;
    status?: {
        confirmed: boolean;
        block_height: number;
        block_hash: string;
        block_time: number;
    };
    value: number;
}


function encode_issue(
    p: string,
    bitvm: string
) {
  const opReturnOutput = payments.embed({
    data: [
      Buffer.from(p, "utf-8"),
      Buffer.from(bitvm),
    ],
  }).output;
  return opReturnOutput;
}
