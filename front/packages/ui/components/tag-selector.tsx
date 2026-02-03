import { useState } from "react"

import "@repo/ui/styles/components/tag-selector.css"

export type Tag = {
  id: string,
  display: string
}

const Tag: React.FunctionComponent<{ 
  tag: Tag,
  selected: boolean
  onClick: (id: string) => void
}> = ({ tag, selected, onClick }) => {

  return (
    <button 
      className={`tag ${selected?'tag--selected': ''}`} 
      onClick={() => onClick(tag.id)} 
      type="button">
      {tag.display}
    </button>
  )
}

const TagSelector: React.FunctionComponent<{
  tags: Tag[],
  name: string
}> = ({tags, name}) => {

  const [selected, setSelected] = useState<string[]>([])
  
  const isSelected = (id: string) => selected.includes(id)

  const handleOnTagClick = (id: string) => {
    const newSelected = selected.slice(0)
    const foundIndex = newSelected.findIndex((e) => e === id)
    if (foundIndex !== -1) {
      newSelected.splice(foundIndex, 1)
    } else {
      newSelected.push(id)
    }
    setSelected(newSelected)
  }

  return (
    <div className="tag-selector" >
      <input className="tag-selector__value" type="text" name={name} id={name} value={selected.join(",")} />
      { tags.map(t => (
        <Tag 
          onClick={handleOnTagClick} 
          selected={isSelected(t.id)} 
          tag={t} 
          key={t.id} />
      )) }
    </div>
  )
}

export default TagSelector