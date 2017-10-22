import React from 'react'
import { connect } from 'react-redux'
class TextInput extends React.Component{
	constructor(){
		super()
		this.onSubmit = this.onSubmit.bind(this)
	}
	render(){
		return(
			<form onSubmit={this.onSubmit}>
				<input type="text" id="input"></input>
				<button type="submit">Send</button>
			</form>
		)
	}
	onSubmit(evt){
		evt.preventDefault()
		this.props.dispatch({type:'SEND_LINE',line:evt.target[0].value})
		console.log(evt.target[0].value)
	}
}

TextInput = connect()(TextInput)
export default TextInput