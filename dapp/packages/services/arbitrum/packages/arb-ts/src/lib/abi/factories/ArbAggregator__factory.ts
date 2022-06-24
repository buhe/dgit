/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */

import { Contract, Signer } from 'ethers'
import { Provider } from '@ethersproject/providers'

import type { ArbAggregator } from '../ArbAggregator'

export class ArbAggregator__factory {
  static connect(
    address: string,
    signerOrProvider: Signer | Provider
  ): ArbAggregator {
    return new Contract(address, _abi, signerOrProvider) as ArbAggregator
  }
}

const _abi = [
  {
    inputs: [],
    name: 'getDefaultAggregator',
    outputs: [
      {
        internalType: 'address',
        name: '',
        type: 'address',
      },
    ],
    stateMutability: 'view',
    type: 'function',
  },
  {
    inputs: [
      {
        internalType: 'address',
        name: 'addr',
        type: 'address',
      },
    ],
    name: 'getPreferredAggregator',
    outputs: [
      {
        internalType: 'address',
        name: '',
        type: 'address',
      },
      {
        internalType: 'bool',
        name: '',
        type: 'bool',
      },
    ],
    stateMutability: 'view',
    type: 'function',
  },
  {
    inputs: [
      {
        internalType: 'address',
        name: 'newDefault',
        type: 'address',
      },
    ],
    name: 'setDefaultAggregator',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [
      {
        internalType: 'address',
        name: 'prefAgg',
        type: 'address',
      },
    ],
    name: 'setPreferredAggregator',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
]
