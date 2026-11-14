import { Droplet, Eye } from 'lucide-react'
import { MovementIcon } from '../components/MovementIcon'

export const categories = {
  eye: { title: 'Eye break', plural: 'Eye breaks', icon: Eye, description: 'Look at something far away for 20 seconds.', action: 'Take eye break' },
  movement: { title: 'Movement break', plural: 'Movement breaks', icon: MovementIcon, description: 'Stand, stretch gently, or take a short walk.', action: 'Take movement break' },
  water: { title: 'Water reminder', plural: 'Water reminders', icon: Droplet, description: 'Consider a drink of water. Follow your own needs and any fluid advice from your clinician.', action: 'Open water reminder' },
} as const
