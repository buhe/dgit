// Code generated by protoc-gen-tstypes. DO NOT EDIT.

export interface SendTransactionArgs {
  signedTransaction?: string
}

export interface SendTransactionReply {
  transactionHash?: string
}

export interface TxAggregatorService {
  SendTransaction: (r: SendTransactionArgs) => SendTransactionReply
}
