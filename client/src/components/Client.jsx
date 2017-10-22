import React from 'react'
import LineBuffer from './linebuffer.jsx'
import TextInput from './textinput.jsx'
import { connect } from 'react-redux'

class Client extends React.Component{
	constructor(){
		super()
		this.setConnected = this.setConnected.bind(this)
		this.onMessage = this.onMessage.bind(this)
		this.state = {
			connected:false,
			lines:[]
		}
		this.webSocket = new WebSocket('ws://' + window.location.hostname +':3012')
		this.webSocket.onopen = this.setConnected
		this.webSocket.onmessage = this.onMessage
	}
	render(){
		return (
			<div>
				<LineBuffer/>
				<TextInput send={this.send}/>
			</div>
		)
	}
	setConnected(){
		this.setState({connected:true})
	}
	onMessage(evt){
		this.props.dispatch({type:'ADD_LINE',line:evt.data})
	}
	componentWillReceiveProps(newProps){
		console.log('sent')
		if (newProps.sendLine != null && newProps.sendLine !== ''){
			let toSend = newProps.sendLine + ''
			this.webSocket.send(toSend)
			this.props.dispatch({type:'SENT_LINE'})
		}
	}
}

function mapStateToProps(state){
	return {
		lines: state.lines,
		sendLine: state.sendLine,
	}
}
Client = connect(mapStateToProps)(Client)
export default Client