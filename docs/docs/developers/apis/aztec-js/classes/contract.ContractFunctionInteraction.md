---
id: "contract.ContractFunctionInteraction"
title: "Class: ContractFunctionInteraction"
sidebar_label: "ContractFunctionInteraction"
custom_edit_url: null
---

[contract](../modules/contract.md).ContractFunctionInteraction

This is the class that is returned when calling e.g. `contract.methods.myMethod(arg0, arg1)`.
It contains available interactions one can call on a method, including view.

## Hierarchy

- `BaseContractInteraction`

  ↳ **`ContractFunctionInteraction`**

## Constructors

### constructor

• **new ContractFunctionInteraction**(`wallet`, `contractAddress`, `functionDao`, `args`): [`ContractFunctionInteraction`](contract.ContractFunctionInteraction.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `wallet` | [`Wallet`](../modules/account.md#wallet) |
| `contractAddress` | `AztecAddress` |
| `functionDao` | `FunctionAbi` |
| `args` | `any`[] |

#### Returns

[`ContractFunctionInteraction`](contract.ContractFunctionInteraction.md)

#### Overrides

BaseContractInteraction.constructor

## Properties

### args

• `Protected` **args**: `any`[]

___

### contractAddress

• `Protected` **contractAddress**: `AztecAddress`

___

### functionDao

• `Protected` **functionDao**: `FunctionAbi`

___

### pxe

• `Protected` **pxe**: `PXE`

#### Inherited from

BaseContractInteraction.pxe

___

### tx

• `Protected` `Optional` **tx**: `Tx`

#### Inherited from

BaseContractInteraction.tx

___

### txRequest

• `Protected` `Optional` **txRequest**: `TxExecutionRequest`

#### Inherited from

BaseContractInteraction.txRequest

___

### wallet

• `Protected` **wallet**: [`Wallet`](../modules/account.md#wallet)

## Methods

### create

▸ **create**(): `Promise`\<`TxExecutionRequest`\>

Create a transaction execution request that represents this call, encoded and authenticated by the
user's wallet, ready to be simulated.

#### Returns

`Promise`\<`TxExecutionRequest`\>

A Promise that resolves to a transaction instance.

#### Overrides

BaseContractInteraction.create

___

### request

▸ **request**(): `FunctionCall`

Returns an execution request that represents this operation. Useful as a building
block for constructing batch requests.

#### Returns

`FunctionCall`

An execution request wrapped in promise.

___

### send

▸ **send**(`options?`): [`SentTx`](contract.SentTx.md)

Sends a transaction to the contract function with the specified options.
This function throws an error if called on an unconstrained function.
It creates and signs the transaction if necessary, and returns a SentTx instance,
which can be used to track the transaction status, receipt, and events.

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `options` | [`SendMethodOptions`](../modules/contract.md#sendmethodoptions) | An optional object containing 'from' property representing the AztecAddress of the sender. If not provided, the default address is used. |

#### Returns

[`SentTx`](contract.SentTx.md)

A SentTx instance for tracking the transaction status and information.

#### Inherited from

BaseContractInteraction.send

___

### simulate

▸ **simulate**(`options?`): `Promise`\<`Tx`\>

Simulates a transaction execution request and returns a tx object ready to be sent.

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `options` | [`SendMethodOptions`](../modules/contract.md#sendmethodoptions) | optional arguments to be used in the creation of the transaction |

#### Returns

`Promise`\<`Tx`\>

The resulting transaction

#### Inherited from

BaseContractInteraction.simulate

___

### view

▸ **view**(`options?`): `Promise`\<`any`\>

Execute a view (read-only) transaction on an unconstrained function.
This method is used to call functions that do not modify the contract state and only return data.
Throws an error if called on a non-unconstrained function.

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `options` | [`ViewMethodOptions`](../modules/contract.md#viewmethodoptions) | An optional object containing additional configuration for the transaction. |

#### Returns

`Promise`\<`any`\>

The result of the view transaction as returned by the contract function.