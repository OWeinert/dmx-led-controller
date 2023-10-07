using DlclNet.Exceptions;
using DlclRpc;

namespace DlclNet.Models;

public struct Animation : IDtoDerivable<AnimationDTO>
{
    public uint Layer { get; set; }
    public List<IFrame> Frames { get; set; }
    
    public AnimationDTO ToDto()
    {
        var dto = new AnimationDTO();
        dto.Layer = Layer;

        var frameDtos = new List<FrameDTO>();
        foreach (var frame in Frames)
        {
            switch (frame)
            {
                case TimedFrame tFrame:
                {
                    var frames = tFrame.Split();
                    var dtos = frames.ToList().ConvertAll(f => f.ToDto());
                    frameDtos.AddRange(dtos);
                    break;
                }
                case IDtoDerivable<FrameDTO> derivFrame:
                    frameDtos.Add(derivFrame.ToDto());
                    break;
                default:
                    throw new DtoMappingException("", frame.GetType());
            }
        }
        dto.Frames.AddRange(frameDtos);

        return dto;
    }

    public Animation AddFrame(IFrame frame)
    {
        Frames.Add(frame);
        return this;
    }
    
    public Animation AddFrames(IEnumerable<IFrame> frames)
    {
        Frames.AddRange(frames);
        return this;
    }
}