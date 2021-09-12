import React, { useEffect, useState } from 'react'
import { Form, Grid } from 'semantic-ui-react'

import { useSubstrate } from './substrate-lib'
import { TxButton } from './substrate-lib/components'
import _ from 'lodash'
import KittyCards from './KittyCards'

export default function Kitties (props) {
  const { api, keyring } = useSubstrate()
  const { accountPair } = props

  const [kitties, setKitties] = useState([])
  const [kittiesCount, setKittiesCount] = useState(0)
  const [status, setStatus] = useState('')

  const fetchKittiesCount = () => {
    api.query.kittiesModule.kittiesCount(c => {
      if (c > 0) {
        setKittiesCount(_.parseInt(c))
      }
    }).catch(console.error)
  }

  const populateKitties = () => {
    if (kittiesCount > 0) {
      const idArr = [...new Array(kittiesCount).keys()]
      const ownerQuery = api.query.kittiesModule.owner.multi(idArr)
      const kittiesQuery = api.query.kittiesModule.kitties.multi(idArr)
      Promise.all([ownerQuery, kittiesQuery]).then(r => {
        const [_owner, _kitties] = r
        setKitties(_.map(_kitties, (item, i) => {
          return {
            id: i,
            dna: item.unwrap(),
            owner: _owner[i].unwrap().toString()
          }
        }))
      })
    }
  }

  useEffect(fetchKittiesCount, [api, keyring])
  useEffect(populateKitties, [api, kittiesCount])

  return <Grid.Column width={16}>
    <h1>现有 {kittiesCount} 个小毛孩</h1>
    <KittyCards kitties={kitties} accountPair={accountPair} setStatus={setStatus}/>
    <Form style={{ margin: '1em 0' }}>
      <Form.Field style={{ textAlign: 'center' }}>
        <TxButton
          accountPair={accountPair} label='创建小毛孩' type='SIGNED-TX' setStatus={setStatus}
          attrs={{
            palletRpc: 'kittiesModule',
            callable: 'create',
            inputParams: [],
            paramFields: []
          }}
        />
      </Form.Field>
    </Form>
    <div style={{ overflowWrap: 'break-word' }}>{status}</div>
  </Grid.Column>
}
